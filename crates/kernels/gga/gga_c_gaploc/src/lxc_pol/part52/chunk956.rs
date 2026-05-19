//! GGA_C_GAPLOC lxc pol — lxc_pol part 52 (v4rhosigma3_17) CSE chunk 956/1013 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part52_v4rhosigma3_17_chunk956<F: Float>(t3689: F, t7995: F, t12000: F, t2787: F, t14266: F, t599: F, t475: F, t49826: F, t105: F, t1064: F, t14272: F, t14284: F, t14285: F, t169: F, t172: F, t2268: F, t2343: F, t380: F, t419: F, t44411: F, t44413: F, t44416: F, t44420: F, t44423: F, t44425: F, t44435: F, t44437: F, t44439: F, t452: F, t49841: F, t535: F, t6305: F) -> (F, F, F, F, F, F) {
    let t49862 = t7995 * t3689;
    let t49866 = t2787 * t12000;
    let t49873 = t599 * t14266;
    let t49874 = t49873 * t475;
    let t49878 = t49826 * t475;
    let t49891 = t44411 - t44413 + t44416 - t44420 - t44423 + t44425 + F::cast_from(0.1138200265427045984e0_f64) * t6305 * t14272 + F::cast_from(0.1138200265427045984e0_f64) * t2268 * t2343 * t49862 + F::cast_from(0.1138200265427045984e0_f64) * t2268 * t2343 * t49866 + F::cast_from(0.28455006635676149599e-1_f64) * t2268 * t535 * t14284 + F::cast_from(0.56910013271352299198e-1_f64) * t2268 * t2343 * t49874 - F::cast_from(0.85365019907028448797e-1_f64) * t2268 * t1064 * t49878 + F::cast_from(0.28455006635676149599e-1_f64) * t419 * t14285 + F::cast_from(0.28455006635676149599e-1_f64) * t105 * t452 * t49841 * t169 * t172 + F::cast_from(0.37940008847568199465e-1_f64) * t380 * t14285 - t44435 + t44437 + t44439;
    (t49862, t49866, t49873, t49874, t49878, t49891)
}
