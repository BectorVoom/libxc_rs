//! GGA_C_GAPLOC lxc pol — lxc_pol part 43 (v4rhosigma3_8) CSE chunk 879/1072 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part43_v4rhosigma3_8_chunk879(t2268: f64, t2343: f64, t41774: f64, t41778: f64, t12840: f64, t6313: f64, t12807: f64, t6305: f64, t41784: f64, t6320: f64, t39774: f64, t39778: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t42790 = 0.56910013271352299198e-1_f64 * t2268 * t2343 * t41774;
    let t42793 = 0.56910013271352299198e-1_f64 * t2268 * t2343 * t41778;
    let t42795 = 0.1138200265427045984e0_f64 * t6313 * t12840;
    let t42797 = 0.22764005308540919679e0_f64 * t6313 * t12807;
    let t42799 = 0.17073003981405689759e0_f64 * t6305 * t12807;
    let t42802 = 0.17073003981405689759e0_f64 * t2268 * t6320 * t41784;
    let t42803 = 0.23712505529730124666e-2_f64 * t39774;
    let t42804 = 0.47425011059460249332e-2_f64 * t39778;
    (t42790, t42793, t42795, t42797, t42799, t42802, t42803, t42804)
}
