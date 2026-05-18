//! GGA_C_GAPC lxc pol — lxc_pol part 36 (v4rho2sigma2_15) CSE chunk 1000/1328 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part36_v4rho2sigma2_15_chunk1000<F: Float>(t11535: F, t11541: F, t11547: F, t11550: F, t11552: F, t11555: F, t11559: F, t11562: F, t11564: F, t11567: F, t11570: F, t11572: F, t11574: F, t11581: F, t11584: F, t11592: F, t11595: F, t11599: F, t11602: F, t11605: F) -> F {
    let t12431 = F::new(0.98332751566569010433e-7) * t11535 - F::new(0.4419852458519115466e-7) * t11541 + F::new(0.2845640240200497334e-7) * t11547 - F::new(0.505954834707648426e-7) * t11550 - F::new(0.2318836277704281739e-4) * t11552 - F::new(0.98332751566569010433e-8) * t11555 + F::new(0.65555167711046006955e-8) * t11559 - F::new(0.11594181388521408695e-4) * t11562 + F::new(0.2318836277704281739e-4) * t11564 + F::new(0.43440462632258606772e-4) * t11567 - F::new(0.4637672555408563478e-4) * t11570 + F::new(0.43440462632258606772e-4) * t11572 - F::new(0.69504740211613770836e-3) * t11574 + F::new(0.57920616843011475696e-5) * t11581 - F::new(0.2698871527777777778e-4) * t11584 + F::new(0.19336854506021130164e-7) * t11592 + F::new(0.18115908419564701085e-6) * t11595 + F::new(0.13506074236995523433e-5) * t11599 + F::new(0.27012148473991046866e-5) * t11602 - F::new(0.42206481990611010728e-7) * t11605;
    t12431
}
