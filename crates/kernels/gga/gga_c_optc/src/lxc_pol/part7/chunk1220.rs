//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1220/1272 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk1220<F: Float>(t1028: F, t26881: F, t19: F, t3105: F, t3145: F, t2849: F, t3107: F, t123: F, t1897: F, t2850: F, t4356: F, t3116: F, t8447: F, t8475: F, t9117: F, t1: F, t1121: F, t11943: F, t12042: F, t26138: F, t26141: F, t26143: F, t26276: F, t26860: F, t26872: F, t26878: F, t26880: F, t438: F, t4386: F, t450: F, t8451: F, t8455: F, t8966: F, t8973: F, t9175: F) -> (F, F, F, F) {
    let t26882 = t26881 * t1028;
    let t26887 = t3145 * t3105 * t19;
    let t26888 = t3107 * t2849;
    let t26889 = t1897 * t123;
    let t26890 = t26888 * t26889;
    let t26894 = t4356 * t2850;
    let t26899 = t3116 * t8447 * t8475;
    let t26901 = t9117 * t1028;
    let t26905 = 0.31555836879183594821e0 * t26138 + 0.42074449172244793095e0 * t3116 * t26141 * t26143 + 0.35500316489081544176e-1 * t1121 * t450 * t26860 * t1 * t438 + 0.1420012659563261767e0 * t3116 * t8451 * t8455 - 0.63111673758367189645e-1 * t26872 + 0.48295341609937543636e-1 * t4386 * t12042 * t26276 + 0.24147670804968771819e-1 * t26878 + 0.47242254414539272975e4 * t11943 * t26880 * t26882 + 0.61048523203065534458e2 * t8973 * t26887 * t26890 - 0.30524261601532767229e2 * t8966 * t26887 * t26894 - 0.37867004255020313788e0 * t26899 + 0.28345352648723563784e5 * t9175 * t26880 * t26901;
    (t26889, t26890, t26894, t26905)
}
