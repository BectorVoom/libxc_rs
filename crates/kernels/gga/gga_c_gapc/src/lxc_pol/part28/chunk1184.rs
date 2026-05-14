//! GGA_C_GAPC lxc pol — lxc_pol part 28 (v4rho2sigma2_7) CSE chunk 1184/1213 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part28_v4rho2sigma2_7_chunk1184<F: Float>(t11039: F, t3265: F, t3822: F, t4905: F, t1096: F, t31777: F, t34318: F, t34321: F, t34323: F, t34325: F, t34328: F, t34344: F, t34346: F, t34351: F, t34353: F, t34356: F) -> (F, F, F, F, F, F, F, F, F, F, F, F, F) {
    let t36892 = t3265 * t11039;
    let t36893 = t4905 * t3822;
    let t36894 = t31777 * t1096;
    let t36896 = 0.21135226489492151266e-6 * t34318;
    let t36897 = 0.42206481990611010728e-7 * t34321;
    let t36898 = 0.21103240995305505364e-7 * t34323;
    let t36899 = 0.13506074236995523433e-5 * t34325;
    let t36900 = 0.63350674672043801542e-5 * t34328;
    let t36906 = 0.11594181388521408695e-4 * t34344;
    let t36907 = 0.43440462632258606772e-4 * t34346;
    let t36908 = 0.50680539737635041234e-3 * t34351;
    let t36909 = 0.40483072916666666669e-4 * t34353;
    let t36910 = 0.57920616843011475696e-5 * t34356;
    (t36892, t36893, t36894, t36896, t36897, t36898, t36899, t36900, t36906, t36907, t36908, t36909, t36910)
}
