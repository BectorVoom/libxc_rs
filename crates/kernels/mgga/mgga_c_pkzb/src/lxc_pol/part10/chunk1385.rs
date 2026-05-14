//! MGGA_C_PKZB lxc pol — lxc_pol part 10 (v4rho4_2) CSE chunk 1385/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part10_v4rho4_2_chunk1385<F: Float>(t2256: F, t3774: F, t2320: F, t9929: F, t18747: F, t18863: F, t2252: F, t22564: F, t22570: F, t2259: F, t22627: F, t22745: F, t2296: F, t2318: F, t27509: F, t27723: F, t27736: F, t27750: F, t27763: F, t3121: F, t3140: F, t3793: F, t3796: F, t6266: F, t6300: F, t6303: F, t8071: F, t8107: F, t8161: F, t8171: F, t863: F, t870: F, t871: F, t889: F, t9875: F, t9878: F, t9881: F, t9930: F, t9959: F, t9981: F, t9986: F, t9989: F) -> (F,) {
    let t27771 = t3774 * t2256;
    let t27795 = t9929 * t2320;
    let t27801 = 0.8276162067083744048e4 * t22570 * t22627 * t870 + 1.0 * t6303 * t3793 + 2.0 * t2252 * t9959 + 1.0 * t863 * (t27723 + t27736 + t27750 + t27763) * t871 + 0.32163958997385070134e2 * t18747 * t3796 - t27509 - 2.0 * t27771 * t2259 - 0.46785788981077169656e1 * t22564 * t3121 + 0.69263436422725855034e2 * t22745 * t3140 - 0.46785788981077169656e1 * t8071 * t8161 + 0.69263436422725855034e2 * t8107 * t8171 + 0.70178683471615754484e1 * t6300 * t9875 - 0.46785788981077169656e1 * t6266 * t9878 - 0.2077903092681775651e3 * t18863 * t9881 - 0.23392894490538584828e1 * t6266 * t9981 + 0.34631718211362927518e2 * t6300 * t9986 - 0.23392894490538584828e1 * t2296 * t9930 * t889 + 0.34631718211362927518e2 * t2318 * t27795 * t889 + 0.69263436422725855036e2 * t6300 * t9989;
    (t27801,)
}
