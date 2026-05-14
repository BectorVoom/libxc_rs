//! MGGA_C_R2SCAN lxc pol — lxc_pol part 6 (v4rho4_1) CSE chunk 1255/1462 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part6_v4rho4_1_chunk1255<F: Float>(t1020: F, t1022: F, t1024: F, t1026: F, t1028: F, t1030: F, t1310: F, t1312: F, t23556: F, t2410: F, t2426: F, t2430: F, t333: F, t335: F, t337: F, t339: F, t341: F, t343: F, t6709: F, t6711: F, t6715: F, t839: F, t8438: F, t8475: F) -> (F,) {
    let t23635 = 0.4651960215348e2 * t2426 * t1310 + 0.1550653405116e2 * t1028 * t6709 - 0.6532958853792e1 * t8475 * t839 - 0.6532958853792e1 * t2430 * t1310 - 0.2177652951264e1 * t1030 * t6709 + 0.734774460522e2 * t6715 * t1020 - 0.27642340881882e2 * t1312 * t2410 - 0.8704e0 * t6709 * t1020 - 0.26112e1 * t1310 * t2410 - 0.26112e1 * t839 * t8438 - 0.8704e0 * t333 * t23556 - 0.4607056813647e1 * t335 * t23556 + 0.122462410087e2 * t337 * t23556 - 0.957855118103e1 * t339 * t23556 + 0.3101306810232e1 * t341 * t23556 - 0.362942158544e0 * t343 * t23556 + 0.2204323381566e3 * t1022 * t6711 - 0.34482784251708e3 * t1024 * t6711 + 0.18607840861392e3 * t1026 * t6711 - 0.3266479426896e2 * t1028 * t6711;
    (t23635,)
}
