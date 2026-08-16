//! GGA_C_GAPLOC lxc pol — lxc_pol part 43 (v4rhosigma3_8) CSE chunk 1070/1072 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part43_v4rhosigma3_8_chunk1070(t224: f64, t51061: f64, t51063: f64, t51072: f64, t51198: f64, t42496: f64, t42501: f64, t42503: f64, t42506: f64, t42509: f64, t42520: f64, t50930: f64, t50931: f64, t50933: f64, t50934: f64, t50983: f64, t50984: f64, t50985: f64, t50986: f64, t50987: f64, t51074: f64, t51075: f64, t51197: f64) -> f64 {
    let t51201 = t224 * (t51061 + t51063 + t51072 + t51198);
    let t51202 = t42496 + t42501 + t42503 + t42506 - t50930 - t50931 - t50933 + t50934 + t42509 + t50983 + t50984 + t50985 + t50986 - t50987 - t42520 + t51201 - t51074 - t51075 + t51197;
    t51202
}
