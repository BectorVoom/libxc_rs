//! GGA_C_GAPLOC lxc pol — lxc_pol part 52 (v4rhosigma3_17) CSE chunk 722/880 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part52_v4rhosigma3_17_chunk722<F: Float>(t13313: F, t6313: F, t42717: F, t42721: F, t13304: F, t484: F, t2325: F, t36117: F, t882: F, t883: F, t13273: F, t2312: F, t42820: F, t13258: F, t2321: F, t38051: F, t9074: F) -> (F, F, F, F, F, F, F, F, F) {
    let t44489 = 0.1138200265427045984e0 * t6313 * t13313;
    let t44490 = 0.94850022118920498664e-2 * t42717;
    let t44491 = 0.47425011059460249332e-2 * t42721;
    let t44492 = t484 * t13304;
    let t44493 = 0.15808337019820083111e-2 * t44492;
    let t44512 = t882 * t2325 * t883 * t36117;
    let t44513 = 0.11856252764865062333e-2 * t44512;
    let t44514 = t2312 * t13273;
    let t44515 = 0.23712505529730124666e-2 * t44514;
    let t44516 = 0.142275033178380748e-1 * t42820;
    let t44517 = t2312 * t13258;
    let t44518 = 0.11856252764865062333e-2 * t44517;
    let t44520 = t9074 * t38051 * t2321;
    (t44489, t44490, t44491, t44493, t44513, t44515, t44516, t44518, t44520)
}
