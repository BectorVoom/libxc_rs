//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 375/1041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk375<F: Float>(t1355: F, t140: F, t2036: F, t2043: F, t543: F, t5530: F, t5557: F, t5579: F, t5593: F, t5604: F, t5613: F, t5785: F, t5787: F, t5791: F, t5797: F, t5802: F, t5813: F, t5814: F, t5821: F, t5824: F, t5829: F, t5831: F, t5837: F, t5838: F) -> (F,) {
    let t5841 = 0.45306850413028723348e0 * t5785 * t5787 - 0.27369475924647479994e0 * t2036 * t5791 + 0.10947790369858991997e1 * t543 * t5557 - 0.22653425206514361674e0 * t2043 * t5797 - 0.12081826776807659559e1 * t543 * t5530 - 0.45306850413028723348e0 * t5802 * t5787 - 0.54738951849294959987e0 * t140 * t5557 + 0.22653425206514361674e0 * t1355 * t5797 + 0.12081826776807659559e1 * t140 * t5530 - 0.10001700163888888889e0 * t5813 * t5579 * t5814 + 0.12083880885367433483e0 * t5821 * t5593 - 0.12083880885367433483e0 * t5824 * t5593 + 0.10001700163888888889e0 * t5829 * t5831 + 0.13335600218518518519e0 * t1355 * t5604 - t5837 - 0.16669500273148148149e-1 * t5838 * t5613;
    (t5841,)
}
