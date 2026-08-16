//! GGA_C_GAPLOC lxc pol — lxc_pol part 38 (v4rhosigma3_3) CSE chunk 842/1003 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part38_v4rhosigma3_3_chunk842<F: Float>(t36178: F, t874: F, t2268: F, t2343: F, t13268: F, t6313: F, t13327: F, t6305: F, t13313: F, t42717: F, t42721: F, t13304: F, t484: F) -> (F, F, F, F, F, F, F, F) {
    let t44480 = t36178 * t874;
    let t44483 = F::cast_from(0.56910013271352299198e-1_f64) * t2268 * t2343 * t44480;
    let t44485 = F::cast_from(0.45528010617081839357e0_f64) * t6313 * t13268;
    let t44487 = F::cast_from(0.28455006635676149599e-1_f64) * t6305 * t13327;
    let t44489 = F::cast_from(0.1138200265427045984e0_f64) * t6313 * t13313;
    let t44490 = F::cast_from(0.94850022118920498664e-2_f64) * t42717;
    let t44491 = F::cast_from(0.47425011059460249332e-2_f64) * t42721;
    let t44492 = t484 * t13304;
    (t44480, t44483, t44485, t44487, t44489, t44490, t44491, t44492)
}
