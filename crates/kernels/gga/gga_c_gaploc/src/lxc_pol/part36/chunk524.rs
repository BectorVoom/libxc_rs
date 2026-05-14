//! GGA_C_GAPLOC lxc pol — lxc_pol part 36 (v4rhosigma3_1) CSE chunk 524/884 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part36_v4rhosigma3_1_chunk524<F: Float>(t3307: F, t9420: F, t813: F, t3209: F, t5750: F, t723: F, t1445: F, t9595: F, t9730: F, t9953: F, t9603: F, t3280: F, t549: F, t2033: F, t325: F, t40: F) -> (F, F, F, F, F, F, F, F, F) {
    let t9981 = t9420 * t3307;
    let t9982 = t813 * t9981;
    let t9984 = t5750 * t3209;
    let t9985 = t9984 * t723;
    let t9986 = t1445 * t9985;
    let t9989 = t9595 * t723;
    let t9990 = t1445 * t9989;
    let t9993 = t1445 * t9730;
    let t9996 = t9953 * t723;
    let t9997 = t1445 * t9996;
    let t10000 = t9603 * t723;
    let t10001 = t1445 * t10000;
    let t10004 = t549 * t3280;
    let t10006 = 0.59584149919750711116e-1 * t2033 * t10004;
    let t10007 = t40 * t325;
    (t9982, t9986, t9989, t9990, t9993, t9997, t10001, t10006, t10007)
}
