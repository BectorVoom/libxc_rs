//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 389/1189 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk389(t1355: f64, t140: f64, t2036: f64, t2043: f64, t543: f64, t5530: f64, t5557: f64, t5579: f64, t5593: f64, t5604: f64, t5613: f64, t5785: f64, t5787: f64, t5791: f64, t5797: f64, t5802: f64, t5813: f64, t5814: f64, t5821: f64, t5824: f64, t5829: f64, t5831: f64, t5837: f64, t5838: f64) -> f64 {
    let t5841 = 0.45306850413028723348e0_f64 * t5785 * t5787 - 0.27369475924647479994e0_f64 * t2036 * t5791 + 0.10947790369858991997e1_f64 * t543 * t5557 - 0.22653425206514361674e0_f64 * t2043 * t5797 - 0.12081826776807659559e1_f64 * t543 * t5530 - 0.45306850413028723348e0_f64 * t5802 * t5787 - 0.54738951849294959987e0_f64 * t140 * t5557 + 0.22653425206514361674e0_f64 * t1355 * t5797 + 0.12081826776807659559e1_f64 * t140 * t5530 - 0.10001700163888888889e0_f64 * t5813 * t5579 * t5814 + 0.12083880885367433483e0_f64 * t5821 * t5593 - 0.12083880885367433483e0_f64 * t5824 * t5593 + 0.10001700163888888889e0_f64 * t5829 * t5831 + 0.13335600218518518519e0_f64 * t1355 * t5604 - t5837 - 0.16669500273148148149e-1_f64 * t5838 * t5613;
    t5841
}
