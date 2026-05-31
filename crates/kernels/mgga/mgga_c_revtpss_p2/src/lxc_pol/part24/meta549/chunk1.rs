//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1623/1850 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1623<F: Float>(t1553: F, t1555: F, t18592: F, t18599: F, t225: F, t227: F, t229: F, t23148: F, t23227: F, t23235: F, t23238: F, t23241: F, t2638: F, t40231: F, t4415: F, t4416: F, t5962: F, t6006: F, t6010: F, t6013: F, t832: F, t87543: F, t87548: F, t87553: F, t87634: F, t87635: F, t87637: F, t87645: F, t87652: F, t87664: F, t87672: F, t87680: F) -> F {
    let t87713 = -(t87634 + t87635 + t87637 + t87645 + t87652 + t87664 + t87672 + t87680) * t225 * t229 + F::cast_from(12.0_f64) * t23227 * t1555 - F::cast_from(72.0_f64) * t6006 * t6010 + F::cast_from(18.0_f64) * t6006 * t6013 + F::cast_from(240.0_f64) * t1553 * t23235 - F::cast_from(144.0_f64) * t18592 * t23238 + F::cast_from(12.0_f64) * t1553 * t23241 - F::cast_from(360.0_f64) * t227 * t40231 * t87553 + F::cast_from(360.0_f64) * t4415 * t18599 * t5962 - F::cast_from(36.0_f64) * t227 * t2638 * t87548 - F::cast_from(48.0_f64) * t4415 * t4416 * t23148 + F::cast_from(3.0_f64) * t227 * t832 * t87543;
    t87713
}
