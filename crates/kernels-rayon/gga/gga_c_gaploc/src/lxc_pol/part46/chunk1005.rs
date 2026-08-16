//! GGA_C_GAPLOC lxc pol — lxc_pol part 46 (v4rhosigma3_11) CSE chunk 1005/1029 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part46_v4rhosigma3_11_chunk1005(t41408: f64, t43586: f64, t7584: f64, t7585: f64, t10012: f64, t2684: f64, t2925: f64, t9438: f64, t3005: f64, t9800: f64, t9829: f64, t41411: f64, t43950: f64, t43955: f64, t43959: f64, t43961: f64, t43964: f64, t43968: f64, t43972: f64, t43975: f64, t43977: f64, t43980: f64, t43983: f64, t43986: f64, t43989: f64, t43991: f64, t43993: f64) -> f64 {
    let t43994 = 0.19171462976960374838e0_f64 * t41408;
    let t43997 = t7584 * t7585 * t43586;
    let t44001 = t2684 * t9438 * t10012 * t2925;
    let t44002 = 0.15976219147466979032e-1_f64 * t44001;
    let t44004 = t9800 * t3005 * t9829;
    let t44005 = 0.19171462976960374838e1_f64 * t44004;
    let t44006 = 0.23005755572352449806e2_f64 * t43950 - t43955 - t43959 - t43961 - 0.92023022289409799224e1_f64 * t43964 - 0.92023022289409799224e1_f64 * t43968 - t43972 - t43975 + t43977 - t43980 - t43983 + t43986 - t43989 + 0.29792074959875355558e-1_f64 * t43991 - t43993 - t43994 - 0.51123901271894332901e0_f64 * t41411 - 0.23005755572352449806e2_f64 * t43997 + t44002 + t44005;
    t44006
}
