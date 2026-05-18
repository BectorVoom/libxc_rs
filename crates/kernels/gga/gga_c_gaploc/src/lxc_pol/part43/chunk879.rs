//! GGA_C_GAPLOC lxc pol — lxc_pol part 43 (v4rhosigma3_8) CSE chunk 879/1072 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part43_v4rhosigma3_8_chunk879<F: Float>(t2268: F, t2343: F, t41774: F, t41778: F, t12840: F, t6313: F, t12807: F, t6305: F, t41784: F, t6320: F, t39774: F, t39778: F) -> (F, F, F, F, F, F, F, F) {
    let t42790 = F::new(0.56910013271352299198e-1) * t2268 * t2343 * t41774;
    let t42793 = F::new(0.56910013271352299198e-1) * t2268 * t2343 * t41778;
    let t42795 = F::new(0.1138200265427045984e0) * t6313 * t12840;
    let t42797 = F::new(0.22764005308540919679e0) * t6313 * t12807;
    let t42799 = F::new(0.17073003981405689759e0) * t6305 * t12807;
    let t42802 = F::new(0.17073003981405689759e0) * t2268 * t6320 * t41784;
    let t42803 = F::new(0.23712505529730124666e-2) * t39774;
    let t42804 = F::new(0.47425011059460249332e-2) * t39778;
    (t42790, t42793, t42795, t42797, t42799, t42802, t42803, t42804)
}
