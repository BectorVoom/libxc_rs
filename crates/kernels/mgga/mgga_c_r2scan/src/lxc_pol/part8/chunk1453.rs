//! MGGA_C_R2SCAN lxc pol — lxc_pol part 8 (v4rho4_3) CSE chunk 1453/1467 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part8_v4rho4_3_chunk1453<F: Float>(t322: F, t35071: F, t1020: F, t1022: F, t1024: F, t1026: F, t1028: F, t10492: F, t10498: F, t2410: F, t2412: F, t2414: F, t2956: F, t2958: F, t333: F, t335: F, t337: F, t339: F, t341: F, t343: F, t839: F, t9707: F, t9711: F) -> (F,) {
    let t332 = 0.25e1 < t322;
    let t35126 = piecewise3(t332, 0.0, t35071);
    let t35164 = -0.64e0 * t35126 + 0.2204323381566e3 * t1022 * t9711 - 0.34482784251708e3 * t1024 * t9711 + 0.18607840861392e3 * t1026 * t9711 - 0.3266479426896e2 * t1028 * t9711 - 0.26112e1 * t2410 * t2956 - 0.26112e1 * t1020 * t9707 - 0.8704e0 * t839 * t10492 - 0.8704e0 * t333 * t35126 - 0.27642340881882e2 * t2958 * t2410 - 0.4607056813647e1 * t335 * t35126 + 0.734774460522e2 * t839 * t10498 + 0.122462410087e2 * t337 * t35126 - 0.957855118103e1 * t339 * t35126 + 0.3101306810232e1 * t341 * t35126 - 0.362942158544e0 * t343 * t35126 - 0.27642340881882e2 * t2412 * t2956 - 0.27642340881882e2 * t2414 * t2956 - 0.27642340881882e2 * t1022 * t9707;
    (t35164,)
}
