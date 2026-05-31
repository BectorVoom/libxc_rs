//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 1260/1332 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk1260<F: Float>(t1841: F, t3740: F, t1016: F, t1814: F, t6102: F, t997: F, t1896: F, t3670: F, t1165: F, t1180: F, t1743: F, t17821: F, t17826: F, t17831: F, t17837: F, t17851: F, t17853: F, t17855: F, t398: F, t4314: F, t942: F, t945: F) -> (F, F) {
    let t23232 = t3740 * t1841;
    let t23234 = t1016 * t1814;
    let t23241 = t997 * t6102;
    let t23243 = t3670 * t1896;
    let t23250 = F::cast_from(0.85748036236139473944e-3_f64) * t942 * t398 * t1743 * t945 + F::cast_from(0.20007875121765877254e-2_f64) * t23232 - F::cast_from(0.12862205435420921092e-2_f64) * t1180 * t1165 * t23234 * t4314 - F::cast_from(0.80031500487063509016e-2_f64) * t17821 + F::cast_from(0.34299214494455789578e-1_f64) * t17826 + F::cast_from(0.40015750243531754508e-2_f64) * t23241 + F::cast_from(0.11337795902333997111e-1_f64) * t23243 + F::cast_from(0.40015750243531754508e-2_f64) * t17831 + F::cast_from(0.40015750243531754508e-1_f64) * t17837 - F::cast_from(35.0_f64) / F::cast_from(54.0_f64) * t17851 - F::cast_from(35.0_f64) / F::cast_from(54.0_f64) * t17853 - F::cast_from(0.80031500487063509016e-2_f64) * t17855;
    (t23234, t23250)
}
