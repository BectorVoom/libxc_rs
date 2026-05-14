//! MGGA_C_RMGGAC lxc pol — lxc_pol part 15 (v4rho3sigma_6) CSE chunk 796/963 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part15_v4rho3sigma_6_chunk796<F: Float>(t44996: F, t7720: F, t3352: F, t495: F, t515: F, t6522: F, t7230: F, t10024: F, t7255: F, t34803: F, t38866: F, t38870: F, t38873: F, t38874: F, t38876: F, t38882: F, t38887: F, t38889: F, t44977: F, t44982: F, t44986: F, t44990: F, t44994: F) -> (F,) {
    let t44997 = t7720 * t44996;
    let t45002 = t7230 * t3352 * t515 * t6522 * t495;
    let t45004 = t7255 * t10024;
    let t45006 = 0.72042316457491791906e-3 * t38866 - t38870 - t38873 - 0.38422568777328955684e-2 * t38874 + 0.92232789896410962678e-3 * t38876 - 0.1333427903096438929e0 * t34803 + 0.19863479950205658386e-4 * t44977 + t38882 + t38887 + 0.16260079888840015101e-2 * t38889 - 0.18183107769496894485e0 * t44982 - 0.15323255961587222183e-3 * t44986 + 0.30646511923174444366e-3 * t44990 + 0.76616279807936110914e-4 * t44994 - 0.76616279807936110914e-4 * t44997 + 0.31923449919973379548e-4 * t45002 + 0.25538759935978703638e-4 * t45004;
    (t45006,)
}
