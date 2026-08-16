//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 430/1184 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk430(t1208: f64, t231: f64, t6045: f64, t1201: f64, t1472: f64, t292: f64, t4094: f64, t4099: f64, t4104: f64, t5265: f64, t6242: f64, t6249: f64, t6255: f64, t6256: f64, t6774: f64, t6795: f64, t6821: f64, t6829: f64, t6833: f64, t6976: f64, t6980: f64, t6986: f64, t7000: f64, t7006: f64, t7009: f64) -> (f64, f64) {
    let t7012 = t231 * t1208;
    let t7013 = t6045 * t7012;
    let t7020 = 0.45306850413028723348e0_f64 * t4094 * t6976 - 0.27369475924647479994e0_f64 * t5265 * t6980 + 0.10947790369858991997e1_f64 * t1201 * t6795 - 0.22653425206514361674e0_f64 * t4099 * t6986 - 0.12081826776807659559e1_f64 * t1201 * t6774 - 0.45306850413028723348e0_f64 * t4104 * t6976 - 0.54738951849294959987e0_f64 * t292 * t6795 + 0.22653425206514361674e0_f64 * t1472 * t6986 + 0.12081826776807659559e1_f64 * t292 * t6774 - 0.10001700163888888889e0_f64 * t6242 * t7000 + 0.12083880885367433483e0_f64 * t7006 * t6821 - 0.12083880885367433483e0_f64 * t7009 * t6821 + 0.10001700163888888889e0_f64 * t6249 * t7013 + 0.13335600218518518519e0_f64 * t1472 * t6829 - t6255 - 0.16669500273148148149e-1_f64 * t6256 * t6833;
    (t7012, t7020)
}
