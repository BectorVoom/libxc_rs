//! GGA_C_GAPLOC lxc pol — lxc_pol part 36 (v4rhosigma3_1) CSE chunk 1006/1029 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part36_v4rhosigma3_1_chunk1006<F: Float>(t41408: F, t43586: F, t7584: F, t7585: F, t10012: F, t2684: F, t2925: F, t9438: F, t3005: F, t9800: F, t9829: F, t41411: F, t43950: F, t43955: F, t43959: F, t43961: F, t43964: F, t43968: F, t43972: F, t43975: F, t43977: F, t43980: F, t43983: F, t43986: F, t43989: F, t43991: F, t43993: F) -> F {
    let t43994 = F::new(0.19171462976960374838e0) * t41408;
    let t43997 = t7584 * t7585 * t43586;
    let t44001 = t2684 * t9438 * t10012 * t2925;
    let t44002 = F::new(0.15976219147466979032e-1) * t44001;
    let t44004 = t9800 * t3005 * t9829;
    let t44005 = F::new(0.19171462976960374838e1) * t44004;
    let t44006 = F::new(0.23005755572352449806e2) * t43950 - t43955 - t43959 - t43961 - F::new(0.92023022289409799224e1) * t43964 - F::new(0.92023022289409799224e1) * t43968 - t43972 - t43975 + t43977 - t43980 - t43983 + t43986 - t43989 + F::new(0.29792074959875355558e-1) * t43991 - t43993 - t43994 - F::new(0.51123901271894332901e0) * t41411 - F::new(0.23005755572352449806e2) * t43997 + t44002 + t44005;
    t44006
}
