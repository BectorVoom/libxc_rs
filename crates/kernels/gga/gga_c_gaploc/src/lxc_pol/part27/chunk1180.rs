//! GGA_C_GAPLOC lxc pol — lxc_pol part 27 (v4rho2sigma2_10) CSE chunk 1180/1468 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part27_v4rho2sigma2_10_chunk1180<F: Float>(t25841: F, t874: F, t2268: F, t2343: F, t23741: F, t3355: F, t10141: F, t6305: F, t2787: F, t6393: F, t10185: F, t1358: F) -> (F, F, F, F, F, F, F) {
    let t31857 = t25841 * t874;
    let t31860 = F::new(0.56910013271352299198e-1) * t2268 * t2343 * t31857;
    let t31863 = F::new(0.56910013271352299198e-1) * t23741 * t3355;
    let t31865 = F::new(0.1138200265427045984e0) * t6305 * t10141;
    let t31866 = t2787 * t6393;
    let t31869 = F::new(0.56910013271352299198e-1) * t2268 * t2343 * t31866;
    let t31878 = t1358 * t10185;
    (t31857, t31860, t31863, t31865, t31866, t31869, t31878)
}
