//! MGGA_C_RMGGAC lxc pol — lxc_pol part 12 (v4rho3sigma_3) CSE chunk 916/951 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part12_v4rho3sigma_3_chunk916<F: Float>(t36424: F, t36590: F, t36594: F, t36601: F, t36710: F, t41690: F, t41694: F, t41696: F, t41701: F, t41706: F, t41713: F, t41717: F, t41719: F, t41723: F, t41726: F, t41727: F, t41730: F, t4965: F, t530: F, t8804: F) -> (F,) {
    let t41732 = 0.79828278012425390428e-1 * t4965 * t8804 - 0.4726e1 * t530 * t36710 + 0.25538759935978703639e-4 * t41690 - 0.25538759935978703639e-4 * t41694 + 0.1064114997332445985e-4 * t41696 - 0.63846899839946759096e-4 * t41701 - 0.25538759935978703638e-4 * t41706 + 0.18183107769496894486e-1 * t36590 + 0.90915538847484472429e-2 * t36594 - 0.2363e1 * t530 * t36424 + 0.8980681276397856423e0 * t41713 + t41717 - 0.5987120850931904282e-1 * t41719 - t41723 - t41726 + 0.33335697577410973224e-1 * t41727 + 2.0 * t36601 - 0.42564599893297839398e-5 * t41730;
    (t41732,)
}
