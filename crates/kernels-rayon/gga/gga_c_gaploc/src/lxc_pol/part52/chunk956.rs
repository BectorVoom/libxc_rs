//! GGA_C_GAPLOC lxc pol — lxc_pol part 52 (v4rhosigma3_17) CSE chunk 956/1013 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part52_v4rhosigma3_17_chunk956(t3689: f64, t7995: f64, t12000: f64, t2787: f64, t14266: f64, t599: f64, t475: f64, t49826: f64, t105: f64, t1064: f64, t14272: f64, t14284: f64, t14285: f64, t169: f64, t172: f64, t2268: f64, t2343: f64, t380: f64, t419: f64, t44411: f64, t44413: f64, t44416: f64, t44420: f64, t44423: f64, t44425: f64, t44435: f64, t44437: f64, t44439: f64, t452: f64, t49841: f64, t535: f64, t6305: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t49862 = t7995 * t3689;
    let t49866 = t2787 * t12000;
    let t49873 = t599 * t14266;
    let t49874 = t49873 * t475;
    let t49878 = t49826 * t475;
    let t49891 = t44411 - t44413 + t44416 - t44420 - t44423 + t44425 + 0.1138200265427045984e0_f64 * t6305 * t14272 + 0.1138200265427045984e0_f64 * t2268 * t2343 * t49862 + 0.1138200265427045984e0_f64 * t2268 * t2343 * t49866 + 0.28455006635676149599e-1_f64 * t2268 * t535 * t14284 + 0.56910013271352299198e-1_f64 * t2268 * t2343 * t49874 - 0.85365019907028448797e-1_f64 * t2268 * t1064 * t49878 + 0.28455006635676149599e-1_f64 * t419 * t14285 + 0.28455006635676149599e-1_f64 * t105 * t452 * t49841 * t169 * t172 + 0.37940008847568199465e-1_f64 * t380 * t14285 - t44435 + t44437 + t44439;
    (t49862, t49866, t49873, t49874, t49878, t49891)
}
