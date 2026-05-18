//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1328/1451 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk1328<F: Float>(t16648: F, t3927: F, t10935: F, t11326: F, t14630: F, t155: F, t16961: F, t17215: F, t17219: F, t25570: F, t25742: F, t25788: F, t2648: F, t2721: F, t2812: F, t2813: F, t297: F, t312: F, t3608: F, t42177: F, t42878: F, t4942: F, t4947: F, t51916: F, t51927: F, t51930: F, t51935: F, t51938: F, t52037: F, t57530: F, t57640: F, t57674: F, t57846: F, t57857: F, t8114: F, t8134: F, t894: F, t940: F, t953: F) -> (F, F) {
    let t57938 = t16648 * t3927;
    let t57943 = -t25788 + F::new(0.11721316454988582616e4) * t51916 - F::new(0.17581974682482873924e4) * t11326 * t42177 * t17219 - F::new(0.45352564237957702055e6) * t25570 * t52037 * t17215 + F::new(0.75587607063262836759e5) * t25742 * t52037 * t4947 + F::new(0.3029360340401625103e1) * t2721 * t3608 * t57846 - F::new(0.5392791351917231181e5) * t8134 * t4942 * t16961 + F::new(0.59919903910191457566e4) * t8114 * t4942 * t155 * t14630 + F::new(0.23181763972770020946e0) * t51927 + F::new(0.75587607063262836759e5) * t51930 + F::new(0.35163949364965747848e4) * t51935 + F::new(0.26631068404529536697e4) * t51938 + F::new(0.5848048239485271795e1) * t940 * t894 * t312 * t57530 * t297 + F::new(0.69310201356862480534e2) * t2812 * t10935 * t57640 + F::new(0.2339219295794108718e2) * t2812 * t2813 * t57674 - F::new(0.40304563566691357832e-1) * t953 * t894 * t2648 * t57857 + F::new(0.1559479530529405812e2) * t2812 * t2813 * t57938 - F::new(0.779739765264702906e1) * t42878;
    (t57938, t57943)
}
