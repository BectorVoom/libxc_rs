//! MGGA_C_KCISK lxc pol — lxc_pol part 26 (v4rho3sigma_6) CSE chunk 950/1407 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part26_v4rho3sigma_6_chunk950<F: Float>(t25882: F, t3677: F, t2093: F, t5748: F, t3639: F, t1190: F, t7789: F, t12910: F, t7786: F, t3679: F, t7785: F, t5752: F, t13023: F, t7753: F, t13021: F, t1201: F, t25840: F, t25864: F, t25867: F, t25871: F, t25875: F, t25879: F, t25881: F, t3692: F, t7804: F, t7821: F, t7825: F) -> (F, F, F, F, F, F, F, F) {
    let t25884 = 6.0 * t3677 * t25882;
    let t25885 = t2093 * t5748;
    let t25887 = 4.0 * t3639 * t25885;
    let t25888 = t7789 * t1190;
    let t25890 = 0.96490945932906628932e2 * t12910 * t25888;
    let t25891 = t7786 * t1190;
    let t25893 = 2.0 * t3639 * t25891;
    let t25894 = t7785 * t3679;
    let t25895 = t25894 * t1190;
    let t25897 = 0.16081824322151104822e2 * t3677 * t25895;
    let t25898 = t5752 * t5748;
    let t25900 = 0.32163648644302209644e2 * t3677 * t25898;
    let t25901 = t7753 * t13023;
    let t25902 = t25901 * t1190;
    let t25904 = 0.51725014705706168417e3 * t13021 * t25902;
    let t25905 = -0.17315755899375863299e2 * t3692 * t7825 - 0.58482233974552040708e0 * t1201 * t25840 - 0.58482233974552040708e0 * t3692 * t7821 + 0.11696446794910408142e1 * t3692 * t7804 - t25864 - 0.1025389702100779493e4 * t1201 * t25867 + 0.11696446794910408142e1 * t1201 * t25871 + 0.1038945353962551798e3 * t1201 * t25875 - t25879 + t25881 + t25884 - t25887 - t25890 - t25893 + t25897 + t25900 + t25904;
    (t25884, t25887, t25890, t25893, t25897, t25900, t25904, t25905)
}
