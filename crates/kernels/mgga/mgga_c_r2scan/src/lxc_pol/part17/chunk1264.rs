//! MGGA_C_R2SCAN lxc pol — lxc_pol part 17 (v4rho3sigma_7) CSE chunk 1264/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part17_v4rho3sigma_7_chunk1264<F: Float>(t322: F, t44684: F, t44715: F, t44746: F, t44778: F, t44811: F, t44842: F, t44873: F, t12203: F, t40276: F, t3250: F, t3560: F, t374: F, t44541: F, t44544: F, t44548: F, t44551: F, t44554: F, t44558: F, t44560: F, t44562: F, t44566: F, t44570: F, t44574: F, t44576: F, t44579: F) -> (F, F) {
    let t323 = t322 <= F::cast_from(0.0_f64);
    let t331 = t322 <= F::cast_from(0.25e1_f64);
    let t44875 = piecewise5::<F>(t323, t44684, t331, t44715 + t44746 + t44778 + t44811, t44842 + t44873);
    let t44878 = F::cast_from(5.0_f64) / F::cast_from(8.0_f64) * t40276 * t12203;
    let t44879 = t3250 * t3560 + t374 * t44875 - t44541 - t44544 + t44548 - t44551 + t44554 - t44558 + t44560 + t44562 - t44566 + t44570 + t44574 + t44576 - t44579 - t44878;
    (t44878, t44879)
}
