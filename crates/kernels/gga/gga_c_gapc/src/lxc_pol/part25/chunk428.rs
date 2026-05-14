//! GGA_C_GAPC lxc pol — lxc_pol part 25 (v4rho2sigma2_4) CSE chunk 428/1225 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part25_v4rho2sigma2_4_chunk428<F: Float>(t2415: F, t297: F, t19: F, t314: F, t20: F, t1191: F, t2262: F, t2265: F, t2269: F, t2301: F, t2305: F, t2308: F, t2311: F, t2317: F, t2406: F, t2409: F, t2414: F, t771: F, t788: F, t794: F, t795: F, t835: F, t877: F) -> (F, F, F, F) {
    let t2416 = t2415 * t297;
    let t2417 = t314 * t19;
    let t2418 = t2417 * t20;
    let t2419 = t2416 * t2418;
    let t2422 = -t1191 - 0.56366309740899397906e-3 * t2262 * t877 - 0.18788769913633132635e-4 * t2265 * t795 - 0.56366309740899397906e-3 * t771 * t2269 - 0.28183154870449698953e-3 * t771 * t2301 + 0.93943849568165663176e-3 * t2305 * t788 + 0.93943849568165663176e-3 * t2308 * t877 + 0.43840463131810642816e-4 * t2311 * t795 + 0.56366309740899397906e-3 * t835 * t2317 - 0.28183154870449698953e-3 * t835 * t2406 - 0.18788769913633132635e-4 * t794 * t2409 + 0.19798879235883268025e-5 * t2414 * t2419;
    (t2416, t2417, t2418, t2422)
}
