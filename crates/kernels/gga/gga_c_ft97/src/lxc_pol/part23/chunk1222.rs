//! GGA_C_FT97 lxc pol — lxc_pol part 23 (v4rho3sigma_8) CSE chunk 1222/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part23_v4rho3sigma_8_chunk1222<F: Float>(t13469: F, t3751: F, t66633: F, t689: F, t17965: F, t27595: F, t108585: F, t17817: F, t108857: F, t109084: F, t1120: F, t123125: F, t123129: F, t123133: F, t123142: F, t123145: F, t123156: F, t123165: F, t17843: F, t17864: F, t17950: F, t231: F, t232: F, t24265: F, t24346: F, t24361: F, t24372: F, t2441: F, t27500: F, t27566: F, t27609: F, t27696: F, t27704: F, t27717: F, t27730: F, t27733: F, t4635: F, t6035: F, t66384: F, t704: F, t709: F, t79855: F, t79864: F, t79911: F, t79935: F, t96465: F, t96607: F, t96615: F) -> (F, F, F, F) {
    let t123169 = t13469 * t3751;
    let t123173 = t66633 * t689;
    let t123177 = t27595 * t17965;
    let t123181 = t17817 * t108585;
    let t123193 = -0.27568129967481981592e-3 * t123125 * t17843 - 0.27568129967481981592e-3 * t123129 * t17843 - 0.3268136001329198891e-5 * t123133 * t79935 + 0.87299078230359608375e-3 * t27566 * t79864 + 0.85124811172839506172e-2 * t108857 - 0.46509801892875584e-1 * t27704 * t27696 - 0.85124811172839506173e-2 * t27500 * t123142 + 0.17024962234567901235e-1 * t24361 * t6035 * t2441 * t123145 + 0.12768721675925925926e-1 * t24361 * t6035 * t704 * t4635 * t709 + 0.12768721675925925926e-1 * t27500 * t123156 + 0.13362091100383075399e-2 * t96607 * t232 * t79855 + 0.23754828622903245156e-2 * t24265 * t1120 * t17864 + 0.89019191601965515283e-5 * t24372 * t232 * t123165 - 0.29673063867321838428e-4 * t24372 * t232 * t123169 + 0.24710505058474293383e-6 * t96465 * t232 * t123173 + 0.89080607335887169332e-3 * t27609 * t232 * t123177 - 0.20715606998445758511e-4 * t123181 * t96615 * t231 * t66384 - 0.77462893625097599762e-3 * t24346 * t109084 * t17950 + 0.75080154872671831175e-1 * t27717 * t79911 - 4.0 * t27733 * t27730;
    (t123169, t123173, t123177, t123193)
}
