//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 411/1042 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk411<F: Float>(t1208: F, t231: F, t6045: F, t1201: F, t1472: F, t292: F, t4094: F, t4099: F, t4104: F, t5265: F, t6242: F, t6249: F, t6255: F, t6256: F, t6774: F, t6795: F, t6821: F, t6829: F, t6833: F, t6976: F, t6980: F, t6986: F, t7000: F, t7006: F, t7009: F) -> (F, F) {
    let t7012 = t231 * t1208;
    let t7013 = t6045 * t7012;
    let t7020 = 0.45306850413028723348e0 * t4094 * t6976 - 0.27369475924647479994e0 * t5265 * t6980 + 0.10947790369858991997e1 * t1201 * t6795 - 0.22653425206514361674e0 * t4099 * t6986 - 0.12081826776807659559e1 * t1201 * t6774 - 0.45306850413028723348e0 * t4104 * t6976 - 0.54738951849294959987e0 * t292 * t6795 + 0.22653425206514361674e0 * t1472 * t6986 + 0.12081826776807659559e1 * t292 * t6774 - 0.10001700163888888889e0 * t6242 * t7000 + 0.12083880885367433483e0 * t7006 * t6821 - 0.12083880885367433483e0 * t7009 * t6821 + 0.10001700163888888889e0 * t6249 * t7013 + 0.13335600218518518519e0 * t1472 * t6829 - t6255 - 0.16669500273148148149e-1 * t6256 * t6833;
    (t7012, t7020)
}
