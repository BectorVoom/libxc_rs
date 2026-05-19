//! GGA_C_GAPLOC lxc pol — lxc_pol part 18 (v4rho2sigma2_1) CSE chunk 1201/1436 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part18_v4rho2sigma2_1_chunk1201<F: Float>(t10167: F, t29874: F, t10269: F, t4141: F, t10196: F, t3833: F, t10243: F, t6305: F, t26011: F, t3822: F, t894: F, t1063: F, t20395: F, t2343: F, t2787: F) -> (F, F, F, F, F, F) {
    let t32052 = t29874 * t10167;
    let t32053 = F::cast_from(0.71137516589190373998e-2_f64) * t32052;
    let t32055 = F::cast_from(0.63233348079280332441e-2_f64) * t4141 * t10269;
    let t32057 = F::cast_from(0.56910013271352299198e-1_f64) * t3833 * t10196;
    let t32059 = F::cast_from(0.56910013271352299198e-1_f64) * t6305 * t10243;
    let t32062 = F::cast_from(0.56910013271352299198e-1_f64) * t3822 * t894 * t26011;
    let t32066 = F::cast_from(0.1138200265427045984e0_f64) * t1063 * t2343 * t2787 * t20395;
    (t32053, t32055, t32057, t32059, t32062, t32066)
}
