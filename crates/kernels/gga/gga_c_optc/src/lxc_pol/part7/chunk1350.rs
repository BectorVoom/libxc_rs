//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1350/1414 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk1350<F: Float>(t26888: F, t26889: F, t2850: F, t4356: F, t3116: F, t8447: F, t8475: F, t1028: F, t9117: F, t1: F, t1121: F, t11943: F, t12042: F, t26138: F, t26141: F, t26143: F, t26276: F, t26860: F, t26872: F, t26878: F, t26880: F, t26882: F, t26887: F, t438: F, t4386: F, t450: F, t8451: F, t8455: F, t8966: F, t8973: F, t9175: F) -> (F, F, F) {
    let t26890 = t26888 * t26889;
    let t26894 = t4356 * t2850;
    let t26899 = t3116 * t8447 * t8475;
    let t26901 = t9117 * t1028;
    let t26905 = F::cast_from(0.31555836879183594821e0_f64) * t26138 + F::cast_from(0.42074449172244793095e0_f64) * t3116 * t26141 * t26143 + F::cast_from(0.35500316489081544176e-1_f64) * t1121 * t450 * t26860 * t1 * t438 + F::cast_from(0.1420012659563261767e0_f64) * t3116 * t8451 * t8455 - F::cast_from(0.63111673758367189645e-1_f64) * t26872 + F::cast_from(0.48295341609937543636e-1_f64) * t4386 * t12042 * t26276 + F::cast_from(0.24147670804968771819e-1_f64) * t26878 + F::cast_from(0.47242254414539272975e4_f64) * t11943 * t26880 * t26882 + F::cast_from(0.61048523203065534458e2_f64) * t8973 * t26887 * t26890 - F::cast_from(0.30524261601532767229e2_f64) * t8966 * t26887 * t26894 - F::cast_from(0.37867004255020313788e0_f64) * t26899 + F::cast_from(0.28345352648723563784e5_f64) * t9175 * t26880 * t26901;
    (t26890, t26894, t26905)
}
