//! GGA_C_GAPLOC lxc pol — lxc_pol part 27 (v4rho2sigma2_10) CSE chunk 1287/1468 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part27_v4rho2sigma2_10_chunk1287<F: Float>(t2714: F, t8556: F, t3040: F, t7593: F, t7596: F, t7590: F, t16251: F, t2103: F, t3447: F, t10713: F, t4673: F, t10717: F) -> (F, F, F, F, F, F, F) {
    let t33863 = F::cast_from(0.47667319935800568892e0_f64) * t2714 * t8556;
    let t33865 = F::cast_from(0.35750489951850426669e0_f64) * t7593 * t3040;
    let t33867 = F::cast_from(0.71500979903700853338e0_f64) * t7596 * t3040;
    let t33869 = F::cast_from(0.35750489951850426669e0_f64) * t7590 * t3040;
    let t33872 = F::cast_from(0.15889106645266856297e0_f64) * t2103 * t16251 * t3447;
    let t33878 = F::cast_from(0.95334639871601137784e0_f64) * t2103 * t4673 * t10713;
    let t33881 = F::cast_from(0.95334639871601137784e0_f64) * t2103 * t4673 * t10717;
    (t33863, t33865, t33867, t33869, t33872, t33878, t33881)
}
