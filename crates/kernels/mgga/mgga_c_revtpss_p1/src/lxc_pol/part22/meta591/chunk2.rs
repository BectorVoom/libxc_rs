//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 2474/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2474<F: Float>(t11134: F, t11304: F, t15189: F, t15209: F, t15210: F, t15211: F, t18906: F, t18911: F, t18915: F, t18919: F, t18924: F, t18928: F, t18932: F, t18934: F, t18939: F, t18944: F, t18948: F) -> F {
    let t18950 = -t11304 - F::cast_from(4.0_f64) / F::cast_from(27.0_f64) * t11134 - F::cast_from(8.0_f64) / F::cast_from(27.0_f64) * t15189 + t15209 - t15210 + t15211 + F::cast_from(2.0_f64) / F::cast_from(27.0_f64) * t18919 - F::cast_from(10.0_f64) / F::cast_from(27.0_f64) * t18906 + F::cast_from(4.0_f64) / F::cast_from(3.0_f64) * t18911 - F::cast_from(4.0_f64) / F::cast_from(9.0_f64) * t18915 - F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t18924 - F::cast_from(2.0_f64) * t18928 + F::cast_from(4.0_f64) / F::cast_from(3.0_f64) * t18932 + t18934 / F::cast_from(9.0_f64) - F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t18939 + F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t18944 - t18948 / F::cast_from(3.0_f64);
    t18950
}
