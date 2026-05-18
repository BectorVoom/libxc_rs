//! GGA_C_GAPLOC lxc pol — lxc_pol part 33 (v4rho2sigma2_16) CSE chunk 1175/1464 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part33_v4rho2sigma2_16_chunk1175<F: Float>(t10178: F, t6305: F, t6313: F, t20539: F, t2343: F, t2787: F, t3822: F, t21172: F, t2765: F, t1063: F, t20073: F, t25665: F, t894: F) -> (F, F, F, F, F, F) {
    let t31840 = F::new(0.34146007962811379518e0) * t6305 * t10178;
    let t31842 = F::new(0.45528010617081839358e0) * t6313 * t10178;
    let t31846 = F::new(0.1138200265427045984e0) * t3822 * t2343 * t2787 * t20539;
    let t31849 = F::new(0.17073003981405689759e0) * t3822 * t2765 * t21172;
    let t31853 = F::new(0.56910013271352299198e-1) * t1063 * t2343 * t2787 * t20073;
    let t31856 = F::new(0.56910013271352299198e-1) * t1063 * t894 * t25665;
    (t31840, t31842, t31846, t31849, t31853, t31856)
}
