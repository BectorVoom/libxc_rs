//! GGA_C_GAPLOC lxc pol — lxc_pol part 43 (v4rhosigma3_8) CSE chunk 921/923 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part43_v4rhosigma3_8_chunk921<F: Float>(t224: F, t51061: F, t51063: F, t51072: F, t51198: F, t42496: F, t42501: F, t42503: F, t42506: F, t42509: F, t42520: F, t50930: F, t50931: F, t50933: F, t50934: F, t50983: F, t50984: F, t50985: F, t50986: F, t50987: F, t51074: F, t51075: F, t51197: F) -> (F,) {
    let t51201 = t224 * (t51061 + t51063 + t51072 + t51198);
    let t51202 = t42496 + t42501 + t42503 + t42506 - t50930 - t50931 - t50933 + t50934 + t42509 + t50983 + t50984 + t50985 + t50986 - t50987 - t42520 + t51201 - t51074 - t51075 + t51197;
    (t51202,)
}
