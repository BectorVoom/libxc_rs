//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1184/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk1184<F: Float>(t1325: F, t51706: F, t14630: F, t4776: F, t16640: F, t3927: F, t1235: F, t297: F, t51636: F, t11473: F, t11518: F, t14635: F, t14640: F, t2721: F, t2722: F, t3608: F, t3884: F, t3907: F, t41994: F, t42129: F, t42145: F, t4768: F, t4962: F, t50937: F, t51027: F, t51102: F, t51349: F, t51355: F, t51360: F, t51363: F, t51368: F, t56718: F, t57599: F, t57628: F, t7397: F, t8002: F, t8214: F, t8215: F, t914: F, t930: F) -> (F, F, F, F, F) {
    let t57670 = t51706 * t1325;
    let t57674 = t14630 * t4776;
    let t57678 = t16640 * t3927;
    let t57701 = t51636 * t297 * t1235;
    let t57708 = -0.12117441361606500412e2 * t2721 * t3608 * t57599 + 0.17581974682482873924e4 * t3884 * t14640 * t51027 - 0.8790987341241436962e3 * t3884 * t14640 * t51102 - 0.11721316454988582616e4 * t3884 * t42129 * t50937 + 0.15146801702008125515e1 * t2721 * t2722 * t57670 + 0.22720202553012188272e1 * t2721 * t2722 * t57674 + 0.90880810212048753088e1 * t2721 * t2722 * t57678 + 0.10431793787746509425e1 * t930 * t914 * t7397 * t57628 - 0.11195712101858710508e-1 * t41994 + 0.15486228121497046737e3 * t3907 * t42145 * t8002 * t4768 - 0.20408653907080965924e7 * t8214 * t14635 * t8215 * t4962 + 0.6717427261115226305e-1 * t51349 + 0.15146801702008125515e1 * t51355 - 0.58606582274942913081e3 * t51360 + 0.15146801702008125515e1 * t51363 - 0.3029360340401625103e1 * t51368 - 0.93568771831764348721e2 * t11473 * t11518 * t57701 - 0.10818156520626009775e1 * t930 * t914 * t56718;
    (t57670, t57674, t57678, t57701, t57708)
}
