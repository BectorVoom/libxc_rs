//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1293/1302 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk1293(t3928: f64, t1167: f64, t1168: f64, t12275: f64, t13707: f64, t18655: f64, t18658: f64, t18667: f64, t18709: f64, t18838: f64, t18914: f64, t2053: f64, t321: f64, t3717: f64, t382: f64, t3929: f64, t43223: f64, t47169: f64, t48495: f64, t48496: f64, t48985: f64, t49019: f64, t49058: f64, t49102: f64, t49147: f64, t49192: f64, t49219: f64, t50389: f64, t50440: f64, t50479: f64, t50514: f64, t50544: f64, t50617: f64, t50642: f64, t50681: f64, t50709: f64, t50737: f64, t804: f64, t8555: f64, t945: f64, t9766: f64) -> f64 {
    let t50744 = t3928 * t3928;
    let t50751 = 72.0_f64 * t9766 * t43223 + t18655 + 18.0_f64 * t804 * t3929 * t3717 + t18658 - t18667 + 24.0_f64 * t13707 * t1168 - 36.0_f64 * t8555 * t12275 * t3928 + t18709 + t18914 + 3.0_f64 * t804 * t382 * t48985 + t321 * (t49019 + t49058 + t49102 + t49147 + t49192 + t49219 + t50389 + t50440 + t50479 + t50514 + t50544 + t50617 + t50642 + t50681 + t50709 + t50737) * t945 - t48495 - t48496 - t18838 - 3.0_f64 * t321 * t50744 * t2053 - 4.0_f64 * t321 * t47169 * t1167;
    t50751
}
