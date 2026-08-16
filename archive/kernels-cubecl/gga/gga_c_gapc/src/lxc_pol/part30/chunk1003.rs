//! GGA_C_GAPC lxc pol — lxc_pol part 30 (v4rho2sigma2_9) CSE chunk 1003/1331 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part30_v4rho2sigma2_9_chunk1003<F: Float>(t11535: F, t11541: F, t11547: F, t11550: F, t11552: F, t11555: F, t11559: F, t11562: F, t11564: F, t11567: F, t11570: F, t11572: F, t11574: F, t11581: F, t11584: F, t11592: F, t11595: F, t11599: F, t11602: F, t11605: F) -> F {
    let t12431 = F::cast_from(0.98332751566569010433e-7_f64) * t11535 - F::cast_from(0.4419852458519115466e-7_f64) * t11541 + F::cast_from(0.2845640240200497334e-7_f64) * t11547 - F::cast_from(0.505954834707648426e-7_f64) * t11550 - F::cast_from(0.2318836277704281739e-4_f64) * t11552 - F::cast_from(0.98332751566569010433e-8_f64) * t11555 + F::cast_from(0.65555167711046006955e-8_f64) * t11559 - F::cast_from(0.11594181388521408695e-4_f64) * t11562 + F::cast_from(0.2318836277704281739e-4_f64) * t11564 + F::cast_from(0.43440462632258606772e-4_f64) * t11567 - F::cast_from(0.4637672555408563478e-4_f64) * t11570 + F::cast_from(0.43440462632258606772e-4_f64) * t11572 - F::cast_from(0.69504740211613770836e-3_f64) * t11574 + F::cast_from(0.57920616843011475696e-5_f64) * t11581 - F::cast_from(0.2698871527777777778e-4_f64) * t11584 + F::cast_from(0.19336854506021130164e-7_f64) * t11592 + F::cast_from(0.18115908419564701085e-6_f64) * t11595 + F::cast_from(0.13506074236995523433e-5_f64) * t11599 + F::cast_from(0.27012148473991046866e-5_f64) * t11602 - F::cast_from(0.42206481990611010728e-7_f64) * t11605;
    t12431
}
