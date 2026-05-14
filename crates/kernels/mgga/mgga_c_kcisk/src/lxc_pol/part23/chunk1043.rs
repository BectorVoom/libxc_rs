//! MGGA_C_KCISK lxc pol — lxc_pol part 23 (v4rho3sigma_3) CSE chunk 1043/1447 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part23_v4rho3sigma_3_chunk1043<F: Float>(t2282: F, t4171: F, t14294: F, t1520: F, t6394: F, t4170: F, t4321: F, t1440: F, t6006: F, t6317: F, t4203: F, t2279: F, t4301: F, t2266: F, t4312: F, t486: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t20931 = t2282 * t4171;
    let t20933 = 6.0 * t14294 * t20931;
    let t20934 = t6394 * t1520;
    let t20936 = 4.0 * t4170 * t20934;
    let t20937 = t2282 * t4321;
    let t20939 = 2.0 * t4170 * t20937;
    let t20940 = t6006 * t1440;
    let t20941 = t6317 * t20940;
    let t20942 = t4203 * t20941;
    let t20944 = t4301 * t2279;
    let t20946 = t4312 * t2266;
    let t20947 = t486 * t20946;
    (t20931, t20933, t20934, t20936, t20937, t20939, t20940, t20942, t20944, t20947)
}
