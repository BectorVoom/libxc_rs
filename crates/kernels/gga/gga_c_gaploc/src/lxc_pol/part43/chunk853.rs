//! GGA_C_GAPLOC lxc pol — lxc_pol part 43 (v4rhosigma3_8) CSE chunk 853/1072 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part43_v4rhosigma3_8_chunk853<F: Float>(t40377: F, t40392: F, t40395: F, t2890: F, t9267: F, t9278: F, t20671: F, t31047: F, t34814: F, t26984: F, t9294: F, t1424: F, t2875: F, t544: F, t9065: F) -> (F, F, F, F, F, F, F) {
    let t42170 = F::new(0.19171462976960374838e0) * t40377;
    let t42172 = F::new(0.15337170381568299871e1) * t40392;
    let t42173 = F::new(0.29792074959875355558e-1) * t40395;
    let t42183 = t9267 * t2890 * t9278;
    let t42184 = F::new(0.19171462976960374838e1) * t42183;
    let t42187 = t31047 * t20671 * t34814;
    let t42188 = F::new(0.42603251059911944084e0) * t42187;
    let t42189 = t26984 * t9294;
    let t42194 = F::new(0.39722766613167140743e-1) * t544 * t9065 * t2875 * t1424;
    (t42170, t42172, t42173, t42184, t42188, t42189, t42194)
}
