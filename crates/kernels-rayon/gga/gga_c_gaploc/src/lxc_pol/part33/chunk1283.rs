//! GGA_C_GAPLOC lxc pol — lxc_pol part 33 (v4rho2sigma2_16) CSE chunk 1283/1464 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part33_v4rho2sigma2_16_chunk1283(t2714: f64, t8556: f64, t3040: f64, t7593: f64, t7596: f64, t7590: f64, t16251: f64, t2103: f64, t3447: f64, t10713: f64, t4673: f64, t10717: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t33863 = 0.47667319935800568892e0_f64 * t2714 * t8556;
    let t33865 = 0.35750489951850426669e0_f64 * t7593 * t3040;
    let t33867 = 0.71500979903700853338e0_f64 * t7596 * t3040;
    let t33869 = 0.35750489951850426669e0_f64 * t7590 * t3040;
    let t33872 = 0.15889106645266856297e0_f64 * t2103 * t16251 * t3447;
    let t33878 = 0.95334639871601137784e0_f64 * t2103 * t4673 * t10713;
    let t33881 = 0.95334639871601137784e0_f64 * t2103 * t4673 * t10717;
    (t33863, t33865, t33867, t33869, t33872, t33878, t33881)
}
