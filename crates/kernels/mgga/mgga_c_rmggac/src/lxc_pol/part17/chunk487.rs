//! MGGA_C_RMGGAC lxc pol — lxc_pol part 17 (v4rho3sigma_8) CSE chunk 487/1111 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part17_v4rho3sigma_8_chunk487<F: Float>(t221: F, t6149: F, t1839: F, t476: F, t209: F, t1835: F, t1195: F, t1467: F, t1500: F, t4505: F, t4544: F, t4556: F, t488: F, t5571: F, t5585: F, t5633: F, t5636: F, t5677: F, t5681: F, t5698: F, t6110: F, t6114: F, t6117: F, t6120: F, t6123: F, t6125: F, t6131: F, t6136: F, t6140: F, t6145: F) -> F {
    let t6150 = t221 * t6149;
    let t6153 = t1839 * t476;
    let t6155 = t221 * t6153 * t209;
    let t6158 = t1835 * t476;
    let t6160 = t221 * t6158 * t209;
    let t6163 = -F::cast_from(0.10975822561044790898e0_f64) * t4544 * t6110 + F::cast_from(0.10975822561044790898e0_f64) * t1467 * t6114 - F::cast_from(0.25610252642437845429e0_f64) * t6117 - t5571 - t5585 - F::cast_from(0.54879112805223954488e-1_f64) * t488 * t6120 - F::cast_from(0.38415378963656768141e0_f64) * t6123 + F::cast_from(0.12805126321218922714e0_f64) * t6125 - F::cast_from(0.76830757927313536284e0_f64) * t5633 + t5636 - F::cast_from(0.85367508808126151427e0_f64) * t5677 + t5681 + F::cast_from(0.54879112805223954488e-1_f64) * t1467 * t6131 - F::cast_from(0.16463733841567186346e0_f64) * t5698 * t6136 + F::cast_from(0.16463733841567186347e0_f64) * t1467 * t6140 - F::cast_from(0.21341877202031537856e0_f64) * t4556 - F::cast_from(0.54879112805223954488e-1_f64) * t1500 * t6145 - F::cast_from(0.27439556402611977244e-1_f64) * t1500 * t6150 - F::cast_from(0.16463733841567186346e0_f64) * t4505 * t6155 + F::cast_from(0.54879112805223954488e-1_f64) * t1195 * t6160;
    t6163
}
