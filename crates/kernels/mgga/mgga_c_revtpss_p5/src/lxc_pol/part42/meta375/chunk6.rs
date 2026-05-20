//! MGGA_C_REVTPSS lxc pol — lxc_pol part 42 (v4rho3tau_5) CSE chunk 1241/1505 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk1241<F: Float>(t11134: F, t11304: F, t15189: F, t15209: F, t15210: F, t15211: F, t18906: F, t18911: F, t18915: F, t18919: F, t18924: F, t18928: F, t18932: F, t18934: F, t18939: F, t18944: F, t18948: F) -> F {
    let t18950 = -t11304 - F::new(4.0) / F::new(27.0) * t11134 - F::new(8.0) / F::new(27.0) * t15189 + t15209 - t15210 + t15211 + F::new(2.0) / F::new(27.0) * t18919 - F::new(10.0) / F::new(27.0) * t18906 + F::new(4.0) / F::new(3.0) * t18911 - F::new(4.0) / F::new(9.0) * t18915 - F::new(2.0) / F::new(9.0) * t18924 - F::new(2.0) * t18928 + F::new(4.0) / F::new(3.0) * t18932 + t18934 / F::new(9.0) - F::new(2.0) / F::new(9.0) * t18939 + F::new(2.0) / F::new(3.0) * t18944 - t18948 / F::new(3.0);
    t18950
}
