//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 1071/1158 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk1071<F: Float>(t34803: F, t38846: F, t38850: F, t38866: F, t38874: F, t38876: F, t38889: F, t42759: F, t42760: F, t42761: F, t42762: F, t42764: F, t42767: F, t42771: F, t42772: F, t44977: F, t44982: F) -> F {
    let t48307 = -F::new(0.17347588262831798123e-3) * t38846 - F::new(0.2881692658299671676e-2) * t38850 - t42759 + t42760 + t42761 + t42762 + F::new(0.1440846329149835838e-2) * t38866 - t42764 - t42767 - F::new(0.7684513755465791136e-2) * t38874 + F::new(0.18446557979282192534e-2) * t38876 - F::new(0.26668558061928778581e0) * t34803 + F::new(0.39726959900411316773e-4) * t44977 + t42771 + t42772 + F::new(0.325201597776800302e-2) * t38889 - F::new(0.36366215538993788973e0) * t44982;
    t48307
}
