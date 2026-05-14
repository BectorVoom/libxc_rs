//! GGA_C_GAPLOC lxc pol — lxc_pol part 49 (v4rhosigma3_14) CSE chunk 805/1028 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part49_v4rhosigma3_14_chunk805<F: Float>(t2478: F, t3358: F, t6576: F, t3177: F, t8272: F, t9267: F, t40208: F, t12953: F, t4781: F, t34478: F, t544: F, t9287: F, t10318: F, t1397: F, t2487: F, t2754: F, t9438: F, t9448: F) -> (F, F, F, F, F, F, F) {
    let t41900 = t6576 * t3358 * t2478;
    let t41903 = t9267 * t8272 * t3177;
    let t41904 = 0.19171462976960374838e1 * t41903;
    let t41905 = 0.10352590007558602413e2 * t40208;
    let t41906 = t4781 * t12953;
    let t41907 = 0.15337170381568299871e1 * t41906;
    let t41909 = t544 * t34478 * t9287;
    let t41914 = t1397 * t10318 * t9287;
    let t41915 = 0.29792074959875355558e-1 * t41914;
    let t41918 = t2487 * t9438 * t9448 * t2754;
    (t41900, t41904, t41905, t41907, t41909, t41915, t41918)
}
