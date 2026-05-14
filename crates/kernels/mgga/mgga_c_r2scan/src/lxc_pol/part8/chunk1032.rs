//! MGGA_C_R2SCAN lxc pol — lxc_pol part 8 (v4rho4_3) CSE chunk 1032/1467 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part8_v4rho4_3_chunk1032<F: Float>(t5433: F, t5437: F, t5441: F, t5444: F, t5451: F, t5454: F, t5459: F, t5463: F, t5467: F, t7745: F, t7753: F, t7756: F, t7776: F, t8937: F, t8942: F, t8946: F, t8948: F, t8950: F) -> (F,) {
    let t10237 = 12.0 * t8937 - 0.32530743900905219526e-1 * t7745 + 0.19518446340543131715e0 * t8942 + t5433 - t5437 + t5441 + 12.0 * t8946 - 0.35089341735807877242e1 * t8948 + 0.51947577317044391277e2 * t8950 + t5444 - 0.1714584e0 * t7753 + 0.24012257405919999999e-1 * t7756 + t5451 + t5454 - t5459 + t5463 + t5467 - 0.33872559466666666666e-2 * t7776;
    (t10237,)
}
