//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1293/1302 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk1293<F: Float>(t3928: F, t1167: F, t1168: F, t12275: F, t13707: F, t18655: F, t18658: F, t18667: F, t18709: F, t18838: F, t18914: F, t2053: F, t321: F, t3717: F, t382: F, t3929: F, t43223: F, t47169: F, t48495: F, t48496: F, t48985: F, t49019: F, t49058: F, t49102: F, t49147: F, t49192: F, t49219: F, t50389: F, t50440: F, t50479: F, t50514: F, t50544: F, t50617: F, t50642: F, t50681: F, t50709: F, t50737: F, t804: F, t8555: F, t945: F, t9766: F) -> F {
    let t50744 = t3928 * t3928;
    let t50751 = F::new(72.0) * t9766 * t43223 + t18655 + F::new(18.0) * t804 * t3929 * t3717 + t18658 - t18667 + F::new(24.0) * t13707 * t1168 - F::new(36.0) * t8555 * t12275 * t3928 + t18709 + t18914 + F::new(3.0) * t804 * t382 * t48985 + t321 * (t49019 + t49058 + t49102 + t49147 + t49192 + t49219 + t50389 + t50440 + t50479 + t50514 + t50544 + t50617 + t50642 + t50681 + t50709 + t50737) * t945 - t48495 - t48496 - t18838 - F::new(3.0) * t321 * t50744 * t2053 - F::new(4.0) * t321 * t47169 * t1167;
    t50751
}
