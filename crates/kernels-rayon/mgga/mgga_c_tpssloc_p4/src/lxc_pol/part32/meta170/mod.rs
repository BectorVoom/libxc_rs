//! MGGA_C_TPSSLOC lxc pol kernel — _part32_v4rho3sigma_8 meta170 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk863;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_meta170(t1043: f64, t2775: f64, t3961: f64, t4582: f64, t2770: f64, t3061: f64, t1615: f64, t376: f64, t1022: f64, t3131: f64, t1023: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t4583, t4584, t4585, t4588, t4589, t4590, t4593, t4594, t4595, t4596, t4599, t4600) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk863(t1043, t2775, t3961, t4582, t2770, t3061, t1615, t376, t1022, t3131, t1023);
    (t4583, t4584, t4585, t4588, t4589, t4590, t4593, t4594, t4595, t4596, t4599, t4600)
}
