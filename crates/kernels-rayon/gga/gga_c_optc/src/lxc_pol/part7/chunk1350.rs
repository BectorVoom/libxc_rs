//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1350/1414 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk1350(t26888: f64, t26889: f64, t2850: f64, t4356: f64, t3116: f64, t8447: f64, t8475: f64, t1028: f64, t9117: f64, t1: f64, t1121: f64, t11943: f64, t12042: f64, t26138: f64, t26141: f64, t26143: f64, t26276: f64, t26860: f64, t26872: f64, t26878: f64, t26880: f64, t26882: f64, t26887: f64, t438: f64, t4386: f64, t450: f64, t8451: f64, t8455: f64, t8966: f64, t8973: f64, t9175: f64) -> (f64, f64, f64) {
    let t26890 = t26888 * t26889;
    let t26894 = t4356 * t2850;
    let t26899 = t3116 * t8447 * t8475;
    let t26901 = t9117 * t1028;
    let t26905 = 0.31555836879183594821e0_f64 * t26138 + 0.42074449172244793095e0_f64 * t3116 * t26141 * t26143 + 0.35500316489081544176e-1_f64 * t1121 * t450 * t26860 * t1 * t438 + 0.1420012659563261767e0_f64 * t3116 * t8451 * t8455 - 0.63111673758367189645e-1_f64 * t26872 + 0.48295341609937543636e-1_f64 * t4386 * t12042 * t26276 + 0.24147670804968771819e-1_f64 * t26878 + 0.47242254414539272975e4_f64 * t11943 * t26880 * t26882 + 0.61048523203065534458e2_f64 * t8973 * t26887 * t26890 - 0.30524261601532767229e2_f64 * t8966 * t26887 * t26894 - 0.37867004255020313788e0_f64 * t26899 + 0.28345352648723563784e5_f64 * t9175 * t26880 * t26901;
    (t26890, t26894, t26905)
}
