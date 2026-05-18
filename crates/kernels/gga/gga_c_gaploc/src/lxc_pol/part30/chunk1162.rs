//! GGA_C_GAPLOC lxc pol — lxc_pol part 30 (v4rho2sigma2_13) CSE chunk 1162/1436 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part30_v4rho2sigma2_13_chunk1162<F: Float>(t10145: F, t6305: F, t10242: F, t1624: F, t2268: F, t10116: F, t6313: F, t10151: F, t1265: F, t23726: F, t3355: F, t10256: F, t590: F) -> (F, F, F, F, F, F, F) {
    let t31522 = F::new(0.1138200265427045984e0) * t6305 * t10145;
    let t31525 = F::new(0.28455006635676149599e-1) * t2268 * t1624 * t10242;
    let t31527 = F::new(0.56910013271352299198e-1) * t6305 * t10116;
    let t31533 = F::new(0.7588001769513639893e-1) * t6313 * t10116;
    let t31534 = t10151 * t1265;
    let t31539 = F::new(0.15176003539027279786e0) * t23726 * t3355;
    let t31540 = t590 * t10256;
    (t31522, t31525, t31527, t31533, t31534, t31539, t31540)
}
