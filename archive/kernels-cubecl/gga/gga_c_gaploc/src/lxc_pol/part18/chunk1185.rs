//! GGA_C_GAPLOC lxc pol — lxc_pol part 18 (v4rho2sigma2_1) CSE chunk 1185/1436 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part18_v4rho2sigma2_1_chunk1185<F: Float>(t10178: F, t6313: F, t20539: F, t2343: F, t2787: F, t3822: F, t21172: F, t2765: F, t1063: F, t20073: F, t25665: F, t894: F) -> (F, F, F, F, F) {
    let t31842 = F::cast_from(0.45528010617081839358e0_f64) * t6313 * t10178;
    let t31846 = F::cast_from(0.1138200265427045984e0_f64) * t3822 * t2343 * t2787 * t20539;
    let t31849 = F::cast_from(0.17073003981405689759e0_f64) * t3822 * t2765 * t21172;
    let t31853 = F::cast_from(0.56910013271352299198e-1_f64) * t1063 * t2343 * t2787 * t20073;
    let t31856 = F::cast_from(0.56910013271352299198e-1_f64) * t1063 * t894 * t25665;
    (t31842, t31846, t31849, t31853, t31856)
}
