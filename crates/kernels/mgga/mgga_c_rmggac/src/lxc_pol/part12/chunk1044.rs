//! MGGA_C_RMGGAC lxc pol — lxc_pol part 12 (v4rho3sigma_3) CSE chunk 1044/1088 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part12_v4rho3sigma_3_chunk1044<F: Float>(t1550: F, t7778: F, t8377: F, t1632: F, t2064: F, t3928: F, t2373: F, t7561: F, t2283: F, t7944: F, t36424: F, t36590: F, t36594: F, t36601: F, t36710: F, t41690: F, t41694: F, t41696: F, t41701: F, t41706: F, t41713: F, t41717: F, t41719: F, t4965: F, t530: F, t8804: F) -> F {
    let t41722 = t1550 * t7778 * t8377;
    let t41723 = F::cast_from(0.15965655602485078085e0_f64) * t41722;
    let t41725 = t3928 * t2064 * t1632;
    let t41726 = F::cast_from(0.47896966807455234256e0_f64) * t41725;
    let t41727 = t2373 * t7561;
    let t41730 = t7944 * t2283;
    let t41732 = F::cast_from(0.79828278012425390428e-1_f64) * t4965 * t8804 - F::new(0.4726e1) * t530 * t36710 + F::cast_from(0.25538759935978703639e-4_f64) * t41690 - F::cast_from(0.25538759935978703639e-4_f64) * t41694 + F::cast_from(0.1064114997332445985e-4_f64) * t41696 - F::cast_from(0.63846899839946759096e-4_f64) * t41701 - F::cast_from(0.25538759935978703638e-4_f64) * t41706 + F::cast_from(0.18183107769496894486e-1_f64) * t36590 + F::cast_from(0.90915538847484472429e-2_f64) * t36594 - F::new(0.2363e1) * t530 * t36424 + F::cast_from(0.8980681276397856423e0_f64) * t41713 + t41717 - F::cast_from(0.5987120850931904282e-1_f64) * t41719 - t41723 - t41726 + F::cast_from(0.33335697577410973224e-1_f64) * t41727 + F::new(2.0) * t36601 - F::cast_from(0.42564599893297839398e-5_f64) * t41730;
    t41732
}
