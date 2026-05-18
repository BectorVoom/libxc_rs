//! GGA_C_GAPC lxc pol — lxc_pol part 37 (v4rho2sigma2_16) CSE chunk 939/1445 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part37_v4rho2sigma2_16_chunk939<F: Float>(t3622: F, t972: F, t1125: F, t2470: F, t9384: F, t9391: F, t9393: F, t9397: F, t9400: F, t9403: F, t9406: F, t9409: F, t9412: F, t9416: F, t9420: F, t9423: F, t9426: F) -> (F, F, F) {
    let t10791 = t3622 * t972;
    let t10794 = t1125 * t2470;
    let t10813 = -F::new(0.28960308421505737848e-5) * t9384 + F::new(0.39476761752968521453e-4) * t9391 - F::new(0.4637672555408563478e-4) * t9393 - F::new(0.29517957899305555558e-5) * t9397 - F::new(0.17989505234049721814e-7) * t9400 + F::new(0.23989005229605304038e-7) * t9403 - F::new(0.17376185052903442709e-3) * t9406 - F::new(0.9275345110817126956e-4) * t9409 - F::new(0.77294542590142724634e-6) * t9412 + F::new(0.33351427252711720978e-8) * t9416 + F::new(0.16413103962948681584e-7) * t9420 + F::new(0.20240885416666666668e-4) * t9423 - F::new(0.10120442708333333334e-4) * t9426;
    (t10791, t10794, t10813)
}
