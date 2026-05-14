//! MGGA_C_REVTPSS lxc pol — lxc_pol part 4 (v3rho3_1) CSE chunk 1054/1274 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part4_v3rho3_1_chunk1054<F: Float>(t3920: F, t5603: F, t2435: F, t5718: F, t1893: F, t2453: F, t3908: F, t1904: F, t3895: F, t2439: F, t213: F, t5710: F, t10157: F, t10160: F, t10163: F, t10166: F, t10169: F, t10176: F, t1445: F, t4071: F, t4078: F, t5715: F, t5775: F) -> (F,) {
    let t14280 = t5603 * t3920;
    let t14290 = t2435 * t5718;
    let t14293 = t2453 * t1893;
    let t14294 = t14293 * t3908;
    let t14296 = t3895 * t1904;
    let t14297 = t2439 * t14296;
    let t14299 = t213 * t5710;
    let t14302 = -t10157 - 0.13009920719177044025e-1 * t14280 - 0.13170898365871023197e1 * t4071 * t5775 + 0.13170898365871023197e1 * t5715 * t4078 - 0.14634331517634470219e-1 * t10160 + 0.13009920719177044025e-2 * t10163 + 0.23131639038696784278e-2 * t10166 + 0.9757440539382783019e-2 * t10169 - 0.73171657588172351096e-2 * t14290 - 0.19514881078765566038e-1 * t10176 + 0.11565819519348392139e-2 * t14294 + 0.65049603595885220126e-3 * t14297 - 0.13170898365871023197e1 * t14299 * t1445;
    (t14302,)
}
