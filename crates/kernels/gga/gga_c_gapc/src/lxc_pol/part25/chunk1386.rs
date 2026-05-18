//! GGA_C_GAPC lxc pol — lxc_pol part 25 (v4rho2sigma2_4) CSE chunk 1386/1444 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part25_v4rho2sigma2_4_chunk1386<F: Float>(t34205: F, t34207: F, t34209: F, t34211: F, t34214: F, t34217: F, t34219: F, t34222: F, t34224: F, t34235: F, t34238: F, t34241: F, t34245: F, t34249: F, t34252: F, t34255: F, t34258: F, t34264: F, t34269: F, t34274: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t36862 = F::new(0.40481770833333333336e-4) * t34205;
    let t36863 = F::new(0.20240885416666666668e-3) * t34207;
    let t36864 = F::new(0.3243554543208642639e-2) * t34209;
    let t36865 = F::new(0.24581606547037760418e-7) * t34211;
    let t36866 = F::new(0.21720231316129303386e-4) * t34214;
    let t36867 = F::new(0.22098551499687900008e-7) * t34217;
    let t36868 = F::new(0.15716489826578034487e-7) * t34219;
    let t36869 = F::new(0.96681162811134562538e-8) * t34222;
    let t36870 = F::new(0.13505639832369200846e-5) * t34224;
    let t36885 = F::new(0.10298285674687440379e-5) * t34235 + F::new(0.40021712703254065175e-8) * t34238 + F::new(0.34752370105806885418e-3) * t34241 + F::new(0.49163213094075520836e-8) * t34245 - F::new(0.16387737698025173612e-8) * t34249 - F::new(0.10298285674687440379e-4) * t34252 - F::new(0.32775475396050347224e-8) * t34255 + F::new(0.22098551499687900008e-7) * t34258 + F::new(0.13259130899812740005e-6) * t34264 - F::new(0.10957198451928583754e-6) * t34269 + F::new(0.78582449132890172432e-8) * t34274;
    (t36862, t36863, t36864, t36865, t36866, t36867, t36868, t36869, t36870, t36885)
}
