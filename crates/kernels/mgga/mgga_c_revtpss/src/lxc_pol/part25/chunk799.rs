//! MGGA_C_REVTPSS lxc pol — lxc_pol part 25 (v4rho3sigma_0) CSE chunk 799/1212 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part25_v4rho3sigma_0_chunk799<F: Float>(t240: F, t9948: F, t247: F, t550: F, t548: F, t4010: F, t72: F, t245: F, t3829: F, t543: F, t3937: F, t1386: F, t820: F, t844: F, t3940: F, t1371: F, t3889: F, t800: F) -> (F, F, F, F, F, F) {
    let t9949 = t9948 * t240;
    let t9951 = t9949 * t550 * t247;
    let t9953 = 0.37792653007779990369e-1 * t548 * t9951;
    let t9954 = t4010 * t72;
    let t9955 = t9954 * t245;
    let t9956 = t543 * t3829;
    let t9958 = t9955 * t3937 * t9956;
    let t9962 = t820 * t1386 * t844;
    let t9963 = t9962 * t3940;
    let t9966 = t800 * t1371 * t3889;
    (t9949, t9951, t9953, t9958, t9963, t9966)
}
