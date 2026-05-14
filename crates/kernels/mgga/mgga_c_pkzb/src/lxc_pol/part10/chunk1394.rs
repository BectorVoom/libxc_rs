//! MGGA_C_PKZB lxc pol — lxc_pol part 10 (v4rho4_2) CSE chunk 1394/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part10_v4rho4_2_chunk1394<F: Float>(t2317: F, t9929: F, t3161: F, t898: F, t2328: F, t9756: F, t10179: F, t3147: F, t8017: F, t10151: F, t10150: F, t6337: F, t22868: F, t3160: F, t3819: F, t6230: F) -> (F, F, F, F, F, F, F, F) {
    let t27984 = t2317 * t9929;
    let t27987 = 0.34631718211362927518e2 * t898 * t27984 * t3161;
    let t27989 = 0.46785788981077169656e1 * t2328 * t9756;
    let t27991 = 0.70178683471615754484e1 * t2328 * t10179;
    let t27993 = 0.2077903092681775651e3 * t3147 * t8017;
    let t27995 = 0.20779030926817756511e3 * t2328 * t10151;
    let t27998 = 0.10389515463408878255e3 * t898 * t10150 * t6337;
    let t28001 = 0.34631718211362927518e2 * t898 * t3160 * t22868;
    let t28002 = t6230 * t3819;
    (t27987, t27989, t27991, t27993, t27995, t27998, t28001, t28002)
}
