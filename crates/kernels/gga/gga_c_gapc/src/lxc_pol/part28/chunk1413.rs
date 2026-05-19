//! GGA_C_GAPC lxc pol — lxc_pol part 28 (v4rho2sigma2_7) CSE chunk 1413/1429 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part28_v4rho2sigma2_7_chunk1413<F: Float>(t34975: F, t34977: F, t34982: F, t34984: F, t34989: F, t34992: F, t34995: F, t35001: F, t35003: F, t35005: F, t35007: F, t35010: F, t35013: F, t35016: F, t35019: F, t35024: F, t35027: F) -> (F, F, F, F, F, F, F) {
    let t37150 = F::cast_from(0.21103240995305505364e-7_f64) * t34975;
    let t37151 = F::cast_from(0.42206481990611010728e-7_f64) * t34977;
    let t37153 = F::cast_from(0.21103240995305505364e-7_f64) * t34982;
    let t37154 = F::cast_from(0.90040494913303489553e-6_f64) * t34984;
    let t37156 = F::cast_from(0.12141398358188788626e-5_f64) * t34989;
    let t37157 = F::cast_from(0.10793703140429833089e-5_f64) * t34992;
    let t37170 = F::cast_from(0.21587406280859666178e-5_f64) * t34995 - F::cast_from(0.24375961217880947793e-4_f64) * t35001 + F::cast_from(0.9275345110817126956e-4_f64) * t35003 - F::cast_from(0.9275345110817126956e-4_f64) * t35005 + F::cast_from(0.99044544404633838508e-5_f64) * t35007 - F::cast_from(0.45020247456651744776e-6_f64) * t35010 - F::cast_from(0.77294542590142724634e-6_f64) * t35013 - F::cast_from(0.13259557375557346398e-6_f64) * t35016 + F::cast_from(0.80045999977926802213e-8_f64) * t35019 - F::cast_from(0.49755503537412447748e-6_f64) * t35024 + F::cast_from(0.61551119569641057312e-8_f64) * t35027;
    (t37150, t37151, t37153, t37154, t37156, t37157, t37170)
}
